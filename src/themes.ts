export interface Theme {
    name: string,
    primary_color: string,
    secondary_color: string,
    tertiary_color: string,
    surface_color: string,
    shadow_color: string,
    text_color: string,
    header_color: string,
    highlights?: string
}

export const Dark: Theme = {
    name: 'Dark',
    primary_color: 'rgb(51, 51, 51)',
    secondary_color: 'rgb(75, 75, 75)',
    tertiary_color: 'gray',
    surface_color: 'black',
    shadow_color: 'white',
    text_color: 'lightgray',
    header_color: 'rgb(118, 118, 118)',
    highlights: 'rgb(30, 30, 30)'
}

export const Light: Theme = {
    name: 'Light',
    primary_color: 'rgba(214, 214, 214, 1)',
    secondary_color: 'rgba(249, 249, 249, 1)',
    tertiary_color: 'rgba(162, 162, 162, 1)',
    surface_color: 'white',
    shadow_color: 'black',
    text_color: 'rgb(51, 51, 51)',
    header_color: 'rgba(114, 114, 114, 1)',
    highlights: 'rgba(185, 185, 185, 1)'
}

export const Whitespace: Theme = {
    name: 'Whitespace',
    primary_color: 'white',
    secondary_color: 'rgb(250, 250, 250)',
    tertiary_color: 'rgb(245,245,245)',
    surface_color: 'rgb(252, 252, 252)',
    shadow_color: 'rgb(247, 247, 247)',
    text_color: 'rgb(240,240,240)',
    header_color: 'rgb(242, 242, 242)',
    highlights: 'rgb(249, 249, 249)'
}

export const Void: Theme = {
    name: 'Void',
    primary_color: 'black',
    secondary_color: 'rgb(5, 5, 5)',
    tertiary_color: 'rgb(10,10,10)',
    surface_color: 'rgb(3, 3, 3)',
    shadow_color: 'rgb(8, 8, 8)',
    text_color: 'rgb(15,15,15)',
    header_color: 'rgb(18, 18, 18)',
    highlights: 'rgb(6, 6, 6)'
}

export const Star: Theme = {
    name: 'STAR',
    primary_color: 'rgba(25, 131, 150, 1)',
    secondary_color: 'rgba(107, 192, 213, 1)',
    tertiary_color: 'rgba(20, 169, 156, 1)',
    surface_color: 'rgba(240, 255, 164, 1)',
    shadow_color: 'rgba(16, 65, 65, 1)',
    text_color: 'rgba(247, 252, 122, 1)',
    header_color: 'rgba(213, 255, 145, 1)',
    highlights: 'rgba(19, 46, 99, 1)'
}