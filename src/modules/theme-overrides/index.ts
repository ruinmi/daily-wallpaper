export const themeOverrides = {
    common: {
        fontFamily: 'Roboto, Arial, sans-serif',
        primaryColor: '#f1f1f1',
    },
    Button: {
        borderRadiusMedium: '8px',
    },
    Input: {
        border: '1px solid var(--outline)',
        borderHover: '1px solid var(--outline-hover)',
        borderFocus: '1px solid var(--outline-focus)',
        borderRadius: '6px',
        color: 'transparent',
        colorFocus: 'transparent',
        boxShadowFocus: 'none',
    },
    ColorPicker: {
        border: '1px solid var(--outline)',
        borderRadius: '6px',
    },
    Select: {
        peers: {
            InternalSelection: {
                color: 'transparent',
                colorActive: 'transparent',
                border: '1px solid var(--outline)',
                borderHover: '1px solid var(--outline-hover)',
                borderFocus: '1px solid var(--outline-focus)',
                borderActive: '1px solid var(--outline-active)',
                boxShadowActive: 'none',
                boxShadowFocus: 'none',
            },
            InternalSelectMenu: {
                color: 'var(--background-menu)'
            },
        }
    },
    TimePicker: {
        panelColor: 'var(--background-menu)',
    }
};