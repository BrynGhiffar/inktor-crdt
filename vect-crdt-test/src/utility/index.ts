const numToHex = (num: number) => {
    const hex = num.toString(16);
    return `${hex.length === 1 ? "0" : ""}${hex}`;
}

export const rgbToHex = (red: number, green: number, blue: number) => {
    return `#${numToHex(red)}${numToHex(green)}${numToHex(blue)}`;
  }