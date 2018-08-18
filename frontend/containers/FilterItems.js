import { connect } from 'react-redux'

import { selectItem, readItem } from '../actions'
import ItemList from '../components/ItemList'

const filterItems = (items, chan_id, channels) => {
  let ret = []
  for (let i of items.values()) {
    if (i.channel_id == chan_id || chan_id == -1) {
      ret.push({channel_title:channels.get(i.channel_id).title, ...i})
    }
  }
  return ret.sort((a,b)=>a.pub_date<b.pub_date)
}

const mapStateToProps = state => ({
  items: filterItems(state.items, state.selectedChannel, state.channels),
  selectedId: state.selectedItem,
  dateNow: state.dateNow,
})

const mapDispatchToProps = dispatch => {
  return {
    onSelectClick: id => {dispatch(selectItem(id))},
    onUnselectClick: () => {dispatch(selectItem())},
    onReadClick: id => {dispatch(readItem(id))},
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(ItemList)
