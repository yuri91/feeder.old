import { connect } from 'react-redux'

import { showModal } from '../actions'
import Modal from '../components/Modal'

const mapStateToProps = state => ({
  show: state.showModal,
})

const mapDispatchToProps = dispatch => {
  return {
    onClose: () => dispatch(showModal(false)),
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Modal)
