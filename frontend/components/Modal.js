import React from 'react'
import PropTypes from 'prop-types'

const Modal = ({ show, onClose, children }) => (
  <div className={"modal" + (show? " modal-show" : "")} onClick={onClose}>
    <div className={"modal-content"} onClick={(e) => e.stopPropagation()}>
      {children}
    </div>
  </div>
)

Modal.propTypes = {
  show: PropTypes.bool.isRequired,
  onClose: PropTypes.func.isRequired
}

export default Modal
