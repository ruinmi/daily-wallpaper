export interface WallpaperCriteria {
    page: number,
    perPage: number,
    keyword: string,
    orientation: string,
    color: string,
    updateHours: number,
    updateMinutes: number,
}

export interface Config {
    activeWpIdx: number,
    activeWpName: string,
    wpChangeDate: string
}