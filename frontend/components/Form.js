import React from 'react'
import PropTypes from 'prop-types'

const onSubmit = (fields, form, onSuccess, onFail, e) => {
  e.preventDefault()
  let validated = {}
  for (let f of fields) {
    if (!f.validate(form[f.name].value)) {
      onFail(f.name)
      return
    }
    validated[f.name] = form[f.name].value
  }
  onSuccess(validated)
}
class Form extends React.Component {
  render() {
    return (
      <form action={"#"} onSubmit={(e) => onSubmit(this.props.fields, this.refs.form, this.props.onSubmitSuccess, this.props.onSubmitFail, e)} className="form" ref="form">
        { this.props.fields.map((f, i) => <div key={i}>{f.descr + ": "}<input type={"text"} name={f.name} /></div>) }
        <input type={"submit"} value={this.props.submitText}/>
      </form>
    )
  }
}

Form.propTypes = {
  fields: PropTypes.arrayOf(
    PropTypes.shape({
      name: PropTypes.string.isRequired,
      descr: PropTypes.string.isRequired,
      validate: PropTypes.func.isRequired,
    }).isRequired
  ).isRequired,
  submitText: PropTypes.string.isRequired,
  onSubmitSuccess: PropTypes.func.isRequired,
  onSubmitFail: PropTypes.func.isRequired,
}

export default Form
