import type { ColorLevels } from '@/stores/colorLevels'
import axios, { AxiosError, type AxiosResponse } from 'axios'

export const getHealth = async (): Promise<AxiosResponse> => {
  return await axios.get('http://localhost:3000').catch(e => { return e })
}

export const postAnalyzeColorLevels = async (
  image: FormDataEntryValue
): Promise<ColorLevels | undefined> => {
  try {
    const result = await axios.post(
      'http://localhost:3000/levels',
      { data: image },
      {
        headers: {
          Accept: 'application/json',
          'Content-Type': 'application/json'
        }
      }
    )
    console.log('Data: ', result.data)
    return result.data

    // Really simple error handling. We just console log the error and return undefined.
  } catch (error: AxiosError | any) {
    if (axios.isAxiosError(error)) {
      console.log(error.code)
      console.log(error.message)
    } else {
      console.log(error)
    }
    return undefined
  }
}

export const postInvertColors = async (
  image: FormDataEntryValue
): Promise<FormDataEntryValue | undefined> => {
  try {
    const result = await axios.post(
      'http://localhost:3000/invert',
      { data: image },
      {
        headers: {
          Accept: 'application/json',
          'Content-Type': 'application/json'
        }
      }
    )
    return result.data

    // Really simple error handling. We just console log the error and return undefined.
  } catch (error: AxiosError | any) {
    if (axios.isAxiosError(error)) {
      console.log(error.code)
      console.log(error.message)
    } else {
      console.log(error)
    }
    return undefined
  }
}
