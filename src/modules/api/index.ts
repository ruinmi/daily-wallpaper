import {convertFileSrc, invoke} from "@tauri-apps/api/core";
import {executor, WallpaperInfo} from "../database";
import {useMainStore} from "../../stores/useMainStore.ts";
import {useCriteriaStore} from "../../stores/useCriteriaStore.ts";
import {exists, remove} from '@tauri-apps/plugin-fs';

export const setWallpaper = async (idx: number) => {
    const mainStore = useMainStore()

    // 设置壁纸
    const wallpaper = mainStore.wallpapers.find(v => v.idx == idx)
    if (!wallpaper) {
        console.error('找不到壁纸')
        return
    }
    await invoke('set_wallpaper_command', {path: wallpaper.location})
    // 已使用
    wallpaper.used = 1
    await executor.update(WallpaperInfo.TABLE_NAME, wallpaper)
    // 记录使用的壁纸
    mainStore.config.activeWpIdx = idx
    mainStore.config.activeWpName = wallpaper.name
}

export const fetchWallpaper = async () => {
    const mainStore = useMainStore()
    const criteriaStore = useCriteriaStore()

    mainStore.displayOverlay = true
    try {
        const {page, perPage, keyword, orientation, color} = criteriaStore.criteria;

        const existingWallpapers = await executor.select(WallpaperInfo.TABLE_NAME, false)
        const existingIds = new Set(existingWallpapers.map((item: any) => item.id))
        const validData: WallpaperInfo[] = []
        let nextPage = page;
        while (validData.length < perPage) {
            nextPage += 1
            const dataArray: any[] = await invoke("fetch_wallpaper", {
                savedPage: page + 1,
                page: nextPage,
                perPage,
                keyword,
                orientation,
                color: color.substring(1),
            })
            const newWallpapers = dataArray.filter((data: any) => !existingIds.has(data.attributes.id))
            validData.push(...newWallpapers)
            // 删除多余的无效数据文件
            if (validData.length > perPage) {
                const extraData = validData.splice(perPage);
                await batchRemoveFiles(extraData.map((data: any) => data.attributes.image.local_file_path))
            }

            // 删除已存在的壁纸文件
            const duplicates = dataArray.filter((data: any) => existingIds.has(data.attributes.id))
            await batchRemoveFiles(duplicates.map((data: any) => data.attributes.image.local_file_path))

            validData.forEach((data: any) => existingIds.add(data.attributes.id))
            // 如果没有更多新数据，提前退出循环
            if (dataArray.length === 0) break;
        }
        const wallpapers = validData
            .map((data: any, idx: number) => {
                const {id, title, image} = data.attributes;
                const imgSrc = convertFileSrc(image.local_file_path)
                return new WallpaperInfo(
                    id,
                    idx + existingWallpapers.length,
                    page + 1,
                    title,
                    image.local_file_path,
                    imgSrc,
                    0
                )
            })

        if (wallpapers && wallpapers.length != 0) {
            await executor.insertBulk(WallpaperInfo.TABLE_NAME, wallpapers)
            // 从零到有，设0
            if (mainStore.wallpapers.length === 0) {
                mainStore.wpIdx = 0
            }
            mainStore.wallpapers.push(...wallpapers)
            criteriaStore.criteria.page = page + 1;
        }
    } catch (e) {
        console.error(e)
    } finally {
        mainStore.displayOverlay = false
    }
}

export const batchRemoveFiles = async (filePaths: string[]) => {
    await Promise.all(filePaths.map(async (path) => {
        if (await exists(path)) {
            console.log(path)
            await remove(path)
        }
    }))
}