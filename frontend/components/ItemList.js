import React from 'react'
import PropTypes from 'prop-types'

import Item from './Item'

const ItemList = ({ items, dateNow, selectedId, onSelectClick, onUnselectClick, onReadClick }) => (
  <div>
    {items.map((item) => (
      <Item key={item.id}
        item={item}
        dateNow={dateNow}
        selected={selectedId==item.id}
        channelTitle={item.channel_title}
        onSelectClick={() => {onSelectClick(item.id); if(!item.read) onReadClick(item.id)}}
        onUnselectClick={() => onUnselectClick(item.id)}
      />
    ))}
  </div>
)

ItemList.propTypes = {
  items: PropTypes.arrayOf(
    PropTypes.shape({
      id: PropTypes.number.isRequired,
      title: PropTypes.string.isRequired,
      description: PropTypes.string.isRequired,
      pub_date: PropTypes.instanceOf(Date).isRequired,
      read: PropTypes.bool.isRequired,
      channel_title: PropTypes.string.isRequired,
    }).isRequired
  ).isRequired,
  dateNow: PropTypes.instanceOf(Date).isRequired,
  selectedId: PropTypes.number.isRequired,
  onSelectClick: PropTypes.func.isRequired,
  onUnselectClick: PropTypes.func.isRequired,
  onReadClick: PropTypes.func.isRequired,
}

export default ItemList
