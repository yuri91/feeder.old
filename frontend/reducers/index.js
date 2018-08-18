import { handleActions, combineActions } from 'redux-actions'

const EMPTY_STATE = {
  selectedChannel: -1,
  selectedItem: -1,
  channels: new Map(),
  items: new Map(),
  dateNow: new Date(),
}

const mergeToMap = (arr, map) => {
  let ret = new Map(map)
  for (let e of arr) {
    ret.set(e.id, e)
  }
  return ret
}

const readItem = (id, items) => {
  let newItems = new Map(items)
  let item = items.get(id)
  newItems.set(id, {...item, read: true})
  return newItems
}

const readAllItems = (items) => {
  let newItems = new Map()
  for (let [id, item] of items) {
    if (item.read == false)
      newItems.set(id, {...item, read: true})
    else
      newItems.set(id, item)
  }
  return newItems
}

export default handleActions(
  {
    TIME_TICK: (state) => ({ ...state, dateNow: new Date()}),
    FILTER_CHANNEL: (state, { payload: id }) => ({ ...state, selectedChannel: id}),
    SELECT_ITEM: (state, { payload: id }) => ({ ...state, selectedItem: id}),
    GET_CHANNELS_FULFILLED: (state, { payload: channels }) => ({ ...state, channels: mergeToMap(channels, state.channels)}),
    GET_ITEMS_FULFILLED: (state, { payload: items }) => ({ ...state, items: mergeToMap(items, state.items)}),
    READ_ITEM_FULFILLED: (state, { payload: id }) => ({ ...state, items: readItem(id, state.items)}),
    READ_ALL_ITEMS_FULFILLED: (state) => ({ ...state, items: readAllItems(state.items)}),
  },
  EMPTY_STATE
)
