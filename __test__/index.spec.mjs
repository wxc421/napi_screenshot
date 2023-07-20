import test from 'ava'

import {screenShots, sum} from '../index.js'

test('sum from native', (t) => {
  screenShots()
  // t.is(sum(1, 2), 3)
})
