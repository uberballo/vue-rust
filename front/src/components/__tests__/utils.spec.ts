import { describe, it, expect } from 'vitest'
import { rgbToHex } from '../../utils'

describe('rgbToHex', () => {
  it('convert rgb values correctly to hex', () => {
    expect(rgbToHex([100, 100, 100])).toBe('#646464')
  })
})
