import { describe, it, expect } from 'vitest'

import { mount } from '@vue/test-utils'
import Greetings from '../Greetings.vue'

describe('HelloWorld', () => {
  it('renders properly', () => {
    const wrapper = mount(Greetings, { props: { msg: 'Hello Vitest' } })
    expect(wrapper.text()).toContain('Project made with')
  })
})
