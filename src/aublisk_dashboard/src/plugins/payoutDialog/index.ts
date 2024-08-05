import { createApp, h } from 'vue'
import PayoutDialogComponent from './PayoutDialog.vue'

interface PayoutDialogOptions {
    emails : string[]
    onConfirm : (data : any) => void,
    token_symbol : string
}
export const usePayoutDialog = () => {
  const showPayoutDialog = (options : PayoutDialogOptions) => {
    const dialogContainer = document.createElement('div')
      document.body.appendChild(dialogContainer)

      const dialogApp = createApp({
        setup() {
          const handleConfirm = (payload : {email : string, amount : Number}) => {
            options.onConfirm(payload)
            dialogApp.unmount()
            document.body.removeChild(dialogContainer)
          }

          const handleCancel = () => {
            dialogApp.unmount()
            document.body.removeChild(dialogContainer)
          }

          const handleClose = () => {
            dialogApp.unmount()
            document.body.removeChild(dialogContainer)
          }

          return () => h(PayoutDialogComponent, {
            emails: options.emails,
            token_symbol: options.token_symbol,
            onConfirm: handleConfirm,
            onCancel: handleCancel,
            onClose: handleClose
          })
        }
      })

      dialogApp.mount(dialogContainer)
  }

  return {
    showPayoutDialog
  }
}

const PayoutDialogPlugin = {
  install(app) {
    app.config.globalProperties.$showPayoutDialog = usePayoutDialog().showPayoutDialog
  }
}

export default PayoutDialogPlugin