const BASE_URL = 'http://localhost:8888/'

const COMMON = {
  mode: 'cors',
  headers: {
    'X-Forwarded-User': 'yuri'
  }
}
export const fetchChannels = () => fetch(BASE_URL+'channels', {
  method: 'get',
  ...COMMON
}).then(r => r.json())

export const fetchItems = () => fetch(BASE_URL+'items?max_items=-1&from_id=-1&to_id=-1', {
  method: 'get',
  ...COMMON
}).then(r => r.json())
  .then(il => {
    il.forEach(i => {
      i.pub_date = new Date(i.pub_date)
    })
    return il
  })

export const readItem = (id) => fetch(BASE_URL+'read/'+id, {
  method: 'post',
  ...COMMON
}).then(() => id)

export const readAllItems = () => fetch(BASE_URL+'read/all', {
  method: 'post',
  ...COMMON
}).then(() => {})

export const addChannel = (url) => fetch(BASE_URL+'subscribe', {
  method: 'post',
  body: JSON.stringify({url: url}),
  ...COMMON
}).then((r) => r.json())
