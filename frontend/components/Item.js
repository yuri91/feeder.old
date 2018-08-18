import React from 'react'
import PropTypes from 'prop-types'

const filterHTML = input => {
  return {__html: input}
}

const timeDelta = (pubDate, now) => {
  let months = now.getMonth() - pubDate.getMonth()
  if (months != 0)
    return months+"mon"
  let days = now.getDate() - pubDate.getDate()
  if (days != 0)
    return days+"d"
  let hours = now.getHours() - pubDate.getHours()
  if (hours != 0)
    return hours+"h"
  let mins = now.getMinutes() - pubDate.getMinutes()
  if (mins != 0)
    return mins+"min"

  return "now"
}

const BriefItem = ({ item, dateNow, channelTitle ,onClick }) => (
  <div className={"brief"+(item.read?" brief-read":"")} onClick={onClick}>
    <span className="brief-channel">{channelTitle}</span>
    <span className="brief-title">{item.title}</span>
    <span className="brief-date">{timeDelta(item.pub_date, dateNow)}</span>
  </div>
)
const DetailedItem = ({ item, onExitClick }) => (
  <div className={"details-content"}>
    <span onClick={onExitClick}>X</span>
    <h4><a href={item.link}>{item.title}</a></h4>
    <div className="details=description" dangerouslySetInnerHTML={filterHTML(item.description)}></div>
    <footer className="details-footer">{item.pub_date.toString()}</footer>
  </div>
)

const Item = ({ item, channelTitle, dateNow, selected, onSelectClick, onUnselectClick }) => {
  if (selected)
    return <DetailedItem item={item} onExitClick={onUnselectClick} />
  else
    return <BriefItem item={item} dateNow={dateNow} channelTitle={channelTitle} onClick={onSelectClick} />
}

Item.propTypes = {
  item: PropTypes.shape({
    title: PropTypes.string.isRequired,
    description: PropTypes.string.isRequired,
    pub_date: PropTypes.instanceOf(Date).isRequired,
    read: PropTypes.bool.isRequired,
  }).isRequired,
  channelTitle: PropTypes.string.isRequired,
  dateNow: PropTypes.instanceOf(Date).isRequired,
  selected: PropTypes.bool.isRequired,
  onSelectClick: PropTypes.func.isRequired,
  onUnselectClick: PropTypes.func.isRequired
}

export default Item
