import {defineStore} from "pinia";
import {WallpaperCriteria} from "../types";
import {Store} from "@tauri-apps/plugin-store";

const PATH = '.criteria.dat'

export const useCriteriaStore = defineStore('criteria-store', () => {
    const criteria = ref<WallpaperCriteria>({} as WallpaperCriteria);
    let store: Store;
    const defaultCriteria: WallpaperCriteria = {
        page: 0,
        perPage: 7,
        keyword: 'nature',
        orientation: 'landscape',
        color: '#000000',
        updateHours: 1,
        updateMinutes: 0
    }

    const init = async () => {
        store = await Store.load(PATH)
        const len = await store.length()
        if (len == 0)
            await create(defaultCriteria)

        await read()
    }

    const create = async (data: WallpaperCriteria) => {
        for (const key in data)
            await store.set(key, data[key as keyof WallpaperCriteria])

        await store.save()
    }

    const read = async () => {
        criteria.value = Object.fromEntries(await store.entries()) as unknown as WallpaperCriteria
    }

    watchDebounced(criteria, async (v) => {
        if (store) {
            for (const key in v)
                await store.set(key, v[key as keyof WallpaperCriteria])

            await store.save()
        }
    }, {
        debounce: 1000,
        deep: true,
    })

    return {criteria, init}
})