export const rules = {
    keyword: {
        required: true,
        trigger: ['blur', 'input'],
        message: '请输入关键字'
    },
    perPage: {
        required: true,
        trigger: ['blur', 'input'],
        type: 'number',
        message: '请输入个数'
    }
}

export const orientationOptions = [
    {
        label: "水平",
        value: "landscape"
    },
    {
        label: "垂直",
        value: "portrait"
    }
]
