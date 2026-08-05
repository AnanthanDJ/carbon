export interface Theme {
  id: string
  name: string
  description: string
  colors: Record<`--${string}`, string>
}
