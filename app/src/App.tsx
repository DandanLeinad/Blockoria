import { RouterProvider } from 'react-router-dom'
import { router } from './router'
import { ToastProvider } from './components/ToastContainer'
import './index.css'

function App() {
  return (
    <ToastProvider maxVisible={5}>
      <RouterProvider router={router} />
    </ToastProvider>
  )
}

export default App
