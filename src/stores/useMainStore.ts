import {defineStore} from "pinia";
import {Store} from "@tauri-apps/plugin-store";
import {Config} from "../types";
import {executor, WallpaperInfo} from "../modules/database";

const PATH = '.main.dat'

export const useMainStore = defineStore('main-store', () => {
    const config = ref<Config>({} as Config)
    const wallpapers = reactive<Array<WallpaperInfo>>([])
    const wpIdx = ref(-1)
    const displayOverlay = ref(false)
    const firstWpIdx = computed(() => {
        if (wallpapers.length > 0) {
            return wallpapers[0].idx
        } else {
            return -1
        }
    })

    const defaultConfig: Config = {
        activeWpIdx: -1,
        activeWpName: '',
        wpChangeDate: ''
    }

    let store: Store;
    const init = async () => {
        store = await Store.load(PATH)
        const len = await store.length()
        if (len == 0)
            await create(defaultConfig)

        await read()

        wallpapers.length = 0
        wallpapers.push(...await executor.select(WallpaperInfo.TABLE_NAME, true))
        wallpapers.sort((a: WallpaperInfo, b: WallpaperInfo) => a.idx - b.idx)
        wpIdx.value = wallpapers.length === 0 ? -1 :
            config.value.activeWpIdx === -1 ? firstWpIdx.value : config.value.activeWpIdx
    }

    const create = async (data: Config) => {
        for (const key in data) {
            await store.set(key, data[key as keyof Config])
        }

        await store.save()
    }

    const read = async () => {
        config.value = Object.fromEntries(await store.entries()) as unknown as Config
    }

    watchDebounced(config, async (v) => {
        if (store) {
            for (const key in v) {
                await store.set(key, v[key as keyof Config])
            }
            await store.save()
        }
    }, {
        debounce: 1000,
        deep: true,
    })

    return {config, wallpapers, wpIdx, displayOverlay, firstWpIdx, init}
})
