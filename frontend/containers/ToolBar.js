import React from 'react'
import PropTypes from 'prop-types'

import { connect } from 'react-redux'
import { readAllItems, getItems, getChannels } from '../actions'

const ToolBar = ({ onReadAllClick, onRefreshClick }) => (
  <div>
    <a href="#" onClick={onReadAllClick} className="site-toolbar-read-all"></a>
    <a href="#" onClick={onRefreshClick} className="site-toolbar-refresh"></a>
  </div>
)

ToolBar.propTypes = {
  onReadAllClick: PropTypes.func.isRequired,
  onRefreshClick: PropTypes.func.isRequired,
}

const mapStateToProps = state => ({
})

const mapDispatchToProps = dispatch => {
  return {
    onReadAllClick: () => dispatch(readAllItems()),
    onRefreshClick: () => {dispatch(getChannels()); dispatch(getItems())},
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(ToolBar)
