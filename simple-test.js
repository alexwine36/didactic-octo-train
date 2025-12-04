const { plus100, sum } = require('./index')

console.assert(plus100(0) === 100, 'Simple test failed')

console.log('From native', sum(40, 2))

console.info('Simple test passed')
