import React from 'react'
import PropTypes from 'prop-types'

import { connect } from 'react-redux'
import { readAllItems, getItems, getChannels, showModal } from '../actions'

const ToolBar = ({ onReadAllClick, onRefreshClick, onAddChannelClick }) => (
  <div>
    <a href="#" onClick={onAddChannelClick} className="site-toolbar-add-channel"></a>
    <a href="#" onClick={onReadAllClick} className="site-toolbar-read-all"></a>
    <a href="#" onClick={onRefreshClick} className="site-toolbar-refresh"></a>
  </div>
)

ToolBar.propTypes = {
  onAddChannelClick: PropTypes.func.isRequired,
  onReadAllClick: PropTypes.func.isRequired,
  onRefreshClick: PropTypes.func.isRequired,
}

const mapStateToProps = state => ({
})

const mapDispatchToProps = dispatch => {
  return {
    onAddChannelClick: () => dispatch(showModal(true)),
    onReadAllClick: () => dispatch(readAllItems()),
    onRefreshClick: () => {dispatch(getChannels()); dispatch(getItems())},
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(ToolBar)
