Появилась возможность создать директорию с файлами классового компонента React.
Пример:
```sh
bem component AppHeader
```
src  
├── components  
│   ├── app-header  
│   │   ├── app-header.jsx  
│   │   ├── app-header.module.scss  
│   │   └── index.js


*app-header.jsx*
```javascript
import React from 'react'
import appHeaderStyles from './app-header.module.css'

class AppHeader extends React.Component {
    constructor(props) {
        super(props)
    }

    render() {

    }
}

export default AppHeader
```
*index.js*
```javascript
export { default } from './app-header'
```

Реэкспорт позволяет в дальнейшем импортировать компонент без дубливания имени
```javascript
import AppHeader from 'src/components/app-header' // вот так
import AppHeader from 'src/components/app-header/app-header' // а не так
```