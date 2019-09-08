const path = require('path')
const tape = require('tape')
const sleep = require('sleep')
const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/HoloFlyer.dna.json")
const dna = Diorama.dna(dnaPath, 'holoflyer')

const diorama = new Diorama({
  instances: {
    alice: dna,
    // bob: dna,
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

diorama.registerScenario("Can create a group", async (s, t, { alice }) => {
  const createResult = await alice.call('publishers', 'create_group', { group: { name: 'food' } })
  console.log(createResult)
  t.notEqual(createResult.Ok, undefined)
})

diorama.registerScenario("Can create and retrieve existing groups", async (s, t, { alice }) => {
  const createResult = await alice.call('publishers', 'create_group', { group: { name: 'food' } })
  console.log(createResult)
  t.notEqual(createResult.Ok, undefined)
  
  const getResult = await alice.call('publishers', 'get_groups', { })
  console.log(getResult)

  t.notEqual(createResult.Ok, undefined)
})

diorama.registerScenario('Can add_publisher some items', async (s, t, { alice }) => {
  const createResult = await alice.call('publishers', 'create_group', { group: { name: 'food' } })
  const listAddr = createResult.Ok

  const result1 = await alice.call('publishers', 'add_publisher', { publisher_item: { name: 'Lidl' }, publisher_addr: listAddr })
  const result2 = await alice.call('publishers', 'add_publisher', { publisher_item: { name: 'Billa' }, publisher_addr: listAddr })

  console.log(result1)
  console.log(result2)

  t.notEqual(result1.Ok, undefined)
  t.notEqual(result2.Ok, undefined)
})

diorama.registerScenario('Can get a list with items', async (s, t, { alice }) => {
  const createResult = await alice.call('publishers', 'create_group', { group: { name: 'food' } })
  const listAddr = createResult.Ok

  await alice.call('publishers', 'add_publisher', { publisher_item: { name: 'Lidl' }, publisher_addr: listAddr })
  sleep.sleep(5)
  await alice.call('publishers', 'add_publisher', { publisher_item: { name: 'Billa' }, publisher_addr: listAddr })

  const getResult = await alice.call('publishers', 'get_publishers', { publisher_addr: listAddr })
  console.log(getResult)

  t.equal(getResult.Ok.items.length, 2, 'there should be 2 items in the list')
})

diorama.run()