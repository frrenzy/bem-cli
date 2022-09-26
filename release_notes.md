Появилась возможность создать директорию с файлами классового компонента React.
Пример:
```sh
bem component AppHeader
```

**При указании в конце параметра `--sass` или `--scss` будет создан модуль с указанным расширением.**

src  
├─ components  
│   ├─ app-header  
│   │   ├─ app-header.jsx  
│   │   ├─ app-header.module.css  
│   │   └─ index.js


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

Реэкспорт позволяет в дальнейшем импортировать компонент без дублирования имени
```javascript
import AppHeader from 'src/components/app-header' // вот так
import AppHeader from 'src/components/app-header/app-header' // а не так
```