import { connect } from 'react-redux'

import { addChannel } from '../actions'
import Form from '../components/Form'

const fields = [
  {
    name: "url",
    descr: "RSS feed URL",
    validate: (v)=>true
  },
]

const mapStateToProps = state => ({
  fields: fields,
  submitText: "Add Channel",
})

const mapDispatchToProps = dispatch => {
  return {
    onSubmitSuccess: ({url}) => dispatch(addChannel(url)),
    onSubmitFail: (f) => {},
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Form)
