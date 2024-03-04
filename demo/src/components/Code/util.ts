import { Color } from "@brynghiffar/vect-crdt-rs";

export const toRgbaString = (
    name: string, 
    color: Color | null
) => {
    if (!color) return "";
    const [red, blue, green, opacity ] = color;
    return ` ${name}=rgba(${red}, ${blue}, ${green}, ${opacity})`;
};