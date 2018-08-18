import { connect } from 'react-redux'

import { filterChannel } from '../actions'
import ChannelList from '../components/ChannelList'

const mapStateToProps = state => ({
  channels: Array.from(state.channels.values()).sort((a,b)=>a.title<b.title),
  currentId: state.selectedChannel
})

const mapDispatchToProps = dispatch => {
  return {
    onChannelClick: id => dispatch(filterChannel(id)),
    onAllClick: () => dispatch(filterChannel())
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(ChannelList)
