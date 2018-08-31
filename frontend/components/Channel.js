import React from 'react'
import PropTypes from 'prop-types'

const Channel = ({ title, current, onClick }) => (
  <a href="#" onClick={onClick} className={current ? 'current' : undefined}>{title}</a>
)

Channel.propTypes = {
  title: PropTypes.string.isRequired,
  current: PropTypes.bool.isRequired,
  onClick: PropTypes.func.isRequired
}

export default Channel
