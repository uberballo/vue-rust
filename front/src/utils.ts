const numberToHex = (value: number): string => {
  const hex = value.toString(16)
  return hex.length == 1 ? '0' + hex : hex
}

// Same as in the backend. Not in use.
export const rgbToHex = (values: number[]): string => {
  const [r, g, b] = values.map((x) => numberToHex(x))
  return '#' + r + g + b
}
