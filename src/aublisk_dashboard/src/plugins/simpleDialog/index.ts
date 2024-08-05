// DialogPlugin.js
import { createApp, h, ref } from 'vue'
import DialogComponent from './Dialog.vue'

interface DialogOptions {
  title : string,
  message : string,
  cancelText: string,
  confirmText : string,
  onConfirm : () => void
  onCancel : () => void
}
export const useDialog = () => {
  const showDialog = (options : DialogOptions) => {
    const dialogContainer = document.createElement('div')
    document.body.appendChild(dialogContainer)

    const dialogApp = createApp({
      setup() {
        const handleConfirm = () => {
          options.onConfirm()
          dialogApp.unmount()
          document.body.removeChild(dialogContainer)
        }

        const handleCancel = () => {
          options.onCancel()
          dialogApp.unmount()
          document.body.removeChild(dialogContainer)
        }

        const handleClose = () => {
          dialogApp.unmount()
          document.body.removeChild(dialogContainer)
        }

        return () => h(DialogComponent, {
          title: options.title,
          message: options.message,
          cancelText: options.cancelText,
          confirmText: options.confirmText,
          onConfirm: handleConfirm,
          onCancel: handleCancel,
          onClose: handleClose
        })
      }
    })

    dialogApp.mount(dialogContainer)
  }

  return {
    showDialog
  }
}

const DialogPlugin = {
    
  install(app : any) {
    app.config.globalProperties.$showDialog = useDialog().showDialog
  }
}

export default DialogPlugin