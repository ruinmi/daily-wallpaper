import {fetchWallpaper, setWallpaper} from "../api";
import {useMainStore} from "../../stores/useMainStore.ts";
import {useCriteriaStore} from "../../stores/useCriteriaStore.ts";

let intervalId: number | null = null;
let isRunning = false;

const shouldChangeWallpaper = (): boolean => {
    const mainStore = useMainStore()
    const criteriaStore = useCriteriaStore()

    const now = new Date()
    const [currentYear, currentMonth, currentDate, currentHours, currentMinutes] = [
        now.getFullYear(),
        now.getMonth() + 1,
        now.getDate(),
        now.getHours(),
        now.getMinutes(),
    ]
    const today = `${currentYear}-${currentMonth}-${currentDate}`;
    const { updateHours, updateMinutes } = criteriaStore.criteria;
    const isTimeToChange =
        currentHours > updateHours ||
        (currentHours === updateHours && currentMinutes >= updateMinutes);
    return mainStore.config.wpChangeDate !== today && isTimeToChange;
}

const changeWallpaper = async () => {
    const mainStore = useMainStore()

    if (mainStore.config.activeWpIdx - mainStore.firstWpIdx >= mainStore.wallpapers.length - 1) {
        await fetchWallpaper()
    }

    await setWallpaper(mainStore.config.activeWpIdx + 1)

    // 更新日期
    const now = new Date()
    mainStore.config.wpChangeDate = `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`
}

export const startWallpaperChanger = () => {
    if (intervalId) {
        console.warn("Wallpaper changer is already running.")
        return
    }

    const executeTask = async () => {
        if (isRunning) return
        isRunning = true

        if (shouldChangeWallpaper()) {
            await changeWallpaper()
        }

        isRunning = false
        if (import.meta.env.DEV) {
            intervalId = setTimeout(executeTask, 1000)
        } else {
            intervalId = setTimeout(executeTask, 60000)
        }
    }

    executeTask()
    console.log("Wallpaper changer started.")
}

export const stopWallpaperChanger = () => {
    if (intervalId) {
        clearTimeout(intervalId)
        intervalId = null
        console.log("Wallpaper changer stopped.")
    }
}