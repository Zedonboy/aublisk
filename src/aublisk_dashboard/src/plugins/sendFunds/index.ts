// src/plugins/sendFundsPlugin.ts

import { App, InjectionKey, createApp, h, inject, ref } from 'vue'
import SendFundsModal from './SendFunds.vue'

export const SendFundsKey: InjectionKey<SendFundsFunction> = Symbol('SendFunds')
type SendFundsFunction = (options: SendFundsOptions) => void
interface SendFundsOptions {
  maxAmount: number
  transferFee: number
  networkFee: number
  currencySymbol: string
  onSend: (amount: number, address: string) => void
}

export const useSendFunds = (): SendFundsFunction => {
    const sendFunds = inject(SendFundsKey)
    if (!sendFunds) {
      throw new Error('SendFundsPlugin not properly installed')
    }
    return sendFunds
  }

export const SendFundsPlugin = {
  install: (app: App) => {
    const sendFunds = (options: SendFundsOptions) => {
      const mountPoint = document.createElement('div')
      document.body.appendChild(mountPoint)

      const isOpen = ref(true)

      const close = () => {
        isOpen.value = false
        setTimeout(() => {
          mountPoint.remove()
          modalApp.unmount()
        }, 300) // Wait for the closing animation
      }

      const modalApp = createApp({
        render() {
          return h(SendFundsModal, {
            isOpen: isOpen.value,
            maxAmount: options.maxAmount,
            transferFee: options.transferFee,
            networkFee: options.networkFee,
            currencySymbol: options.currencySymbol,
            onClose: close,
            onSend: (amount: number, address: string) => {
              options.onSend(amount, address)
              close()
            }
          })
        }
      })

      modalApp.mount(mountPoint)
    }

    app.config.globalProperties.$sendFunds = sendFunds
    app.provide(SendFundsKey, sendFunds)
  }
}

declare module '@vue/runtime-core' {
  interface ComponentCustomProperties {
    $sendFunds: (options: SendFundsOptions) => void
  }
}