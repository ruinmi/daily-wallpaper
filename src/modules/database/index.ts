import Database from "@tauri-apps/plugin-sql";

let db: Database

export async function initDb() {
    if (!db) {
        db = await Database.load('sqlite:data.db')
    }
}

class ModelExecutor {
    private generateInsertSQL<T extends TableModel>(
        tableName: string,
        models: T | T[]
    ): { sql: string; values: any[] } {
        const isArray = Array.isArray(models);
        const modelArray = isArray ? (models as T[]) : [models];

        const columns = Object.keys(modelArray[0]); // 获取列名
        const placeholders = modelArray.map((_, rowIndex) =>
            `(${columns.map((_, colIndex) => `$${rowIndex * columns.length + colIndex + 1}`).join(", ")})`
        );
        const sql = `
            INSERT INTO ${tableName} (${columns.join(", ")})
            VALUES ${placeholders.join(", ")};
        `;
        const values = modelArray.flatMap((model) => Object.values(model)); // 将所有模型的值展平为数组

        return {sql, values};
    }

    async insert<T extends TableModel>(tableName: string, model: T): Promise<boolean> {
        await initDb();

        const { sql, values } = this.generateInsertSQL(tableName, model);

        await db.execute(sql, values)
        return true
    }

    async insertBulk<T extends TableModel>(tableName: string, models: T[]): Promise<void> {
        await initDb();

        const { sql, values } = this.generateInsertSQL(tableName, models);

        await db.execute(sql, values);
    }

    async select(
        tableName: string,
        logical: boolean,
        kv?: Record<string, any>
    ): Promise<any[]> {
        await initDb();
        let sql = `SELECT * FROM ${tableName} WHERE 1 = 1`;

        if (logical) {
            sql += ` AND deleted = 0`
        }

        let values: any[] = []
        if (kv && Object.keys(kv).length > 0) {
            const whereClauses = Object.keys(kv).map((key, idx) => `${key} = $${idx + 1}`)
            sql += ` AND ${whereClauses.join(' AND ')}`
            values = Object.values(kv)
        }
        return await db.select(sql, values);
    }

    async count(tableName: string) {
        await initDb();
        const sql = `SELECT count(*) AS total_count
                     FROM ${tableName}`
        const res: [any] = await db.select(sql)
        return res[0].total_count
    }

    async delete(tableName: string, id: number, logical: boolean) {
        await initDb();
        if (logical) {
            const [pending] = await this.select(tableName, true, {id: id})
            if (pending) {
                pending.deleted = 1
                await this.update(tableName, pending)
            }
        } else {
            const sql = `DELETE
                     FROM ${tableName}
                     WHERE id = ${id}`
            await db.execute(sql)
        }
    }

    async update<T extends TableModel>(tableName: string, model: T) {
        await initDb();

        // 检查 model 是否包含 id
        if (!model.id) {
            throw new Error("The model must include an 'id' property for the update operation.");
        }

        // 获取需要更新的字段（排除 undefined 和 id）
        const keys = Object.keys(model).filter(
            key => model[key as keyof T] !== undefined && key !== 'id'
        );

        // 构建 SET 子句
        const setClauses = keys.map((key, idx) => `${key} = $${idx + 1}`);

        // SQL 语句
        const sql = `
            UPDATE ${tableName}
            SET ${setClauses.join(', ')}
            WHERE id = $${keys.length + 1}`;

        // 绑定的值：更新字段值 + id
        const values = [
            ...keys.map(key => model[key as keyof T]),
            model.id,
        ];

        await db.execute(sql, values);
    }

}

export const executor = new ModelExecutor()

interface TableModel {
    id: number
    deleted: number
}

export class WallpaperInfo implements TableModel {
    static TABLE_NAME = 'wallpaper_info'
    id: number
    idx: number
    page: number
    name: string
    location: string
    used: number
    img_src: string
    deleted: number

    constructor(id: number, idx: number, page: number, name: string, location: string, img_src: string, used: number) {
        this.id = id ?? Date.now() + Math.floor(Math.random() * 1000)
        this.idx = idx
        this.page = page
        this.name = name
        this.location = location
        this.img_src = img_src
        this.used = used
        this.deleted = 0
    }
}