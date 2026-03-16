export function mapToDefaultTerminalFgColor(
  color: string,
  dark: boolean
): string {
  return color === 'white' && !dark ? 'black' : color;
}
