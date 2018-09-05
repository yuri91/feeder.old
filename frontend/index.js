import React from 'react'
import { render } from 'react-dom'
import { Provider } from 'react-redux'
import { createStore, applyMiddleware } from 'redux'
import { createLogger } from 'redux-logger'
import promiseMiddleware from 'redux-promise-middleware'

import './index.css'

import FilterChannels from './containers/FilterChannels'
import FilterItems from './containers/FilterItems'
import ToolBar from './containers/ToolBar'
import Modal from './containers/Modal'

import rootReducer from './reducers'
import { getChannels, getItems, timeTick } from './actions'

const store = createStore(rootReducer, undefined, applyMiddleware(
  promiseMiddleware(),
  createLogger({collapsed: true}),
))

const App = () => (
  <div id="site">
    <header className="site-header">Feeder</header>
    <nav className="site-nav">
      <FilterChannels />
    </nav>
    <div className="site-toolbar">
      <ToolBar />
    </div>
    <main className="site-main">
      <FilterItems />
    </main>
    <footer className="site-footer"></footer>
    <Modal><p>aaa</p></Modal>
  </div>
)

render(
  <Provider store={store}>
    <App />
  </Provider>,
  document.getElementById('root')
)

store.dispatch(getChannels())
store.dispatch(getItems())
setInterval(()=>store.dispatch(timeTick()), 60*1000)
