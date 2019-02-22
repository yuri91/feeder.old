import { createAction } from 'redux-actions'
import * as api from '../api'

export const filterChannel = createAction('FILTER_CHANNEL', (id = -1) => id)
export const selectItem = createAction('SELECT_ITEM', (id = -1) => id)

export const showModal = createAction('SHOW_MODAL', (show = true) => show)

export const timeTick = createAction('TIME_TICK')

export const getChannels = createAction('GET_CHANNELS', api.fetchChannels)
export const getItems = createAction('GET_ITEMS', api.fetchItems)
export const readItem = createAction('READ_ITEM', api.readItem)
export const readAllItems = createAction('READ_ALL_ITEMS', api.readAllItems)
export const addChannel = createAction('ADD_CHANNEL', api.addChannel)
