import React from 'react'
import PropTypes from 'prop-types'

import Channel from './Channel'

const ChannelList = ({ channels, currentId, onChannelClick, onAllClick }) => (
  <div>
    <Channel current={currentId==-1} onClick={onAllClick} title="All Feeds"/>
    {channels.map((channel) => (
      <Channel key={channel.id} current={currentId==channel.id} onClick={() => onChannelClick(channel.id)} title={channel.title} />
    ))}
  </div>
)

ChannelList.propTypes = {
  channels: PropTypes.arrayOf(
    PropTypes.shape({
      title: PropTypes.string.isRequired,
      id: PropTypes.number.isRequired,
    }).isRequired
  ).isRequired,
  currentId: PropTypes.number.isRequired,
  onChannelClick: PropTypes.func.isRequired,
  onAllClick: PropTypes.func.isRequired
}

export default ChannelList
