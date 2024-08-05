// qrModalPlugin.ts
import { ref, h, render, Plugin, App, InjectionKey, createApp } from "vue";
import QRModal from "./QRModal.vue";
// qrCodeGenerator.ts

import QRCode from 'qrcode'

export async function generateQRCode(text: string): Promise<string> {
  try {
    const url = await QRCode.toDataURL(text)
    return url
  } catch (err) {
    console.error('Error generating QR code:', err)
    throw err
  }
}

interface QRModalMethods {
  openModal: (data : {url: string, text: string, onClose : () => void, onSuccess : () => void}) => void;
  closeModal: () => void;
}

export const QRModalKey: InjectionKey<QRModalMethods> = Symbol("QRModal");

const isOpen = ref(false);
const qrCodeUrl = ref("");
const qrText = ref("");

const openModal = (data : {url: string, text: string, onClose : () => void, onSuccess : () => void}): void => {
  qrCodeUrl.value = data.url;
  qrText.value = data.text;
  isOpen.value = true;

  let modalContainer: HTMLElement | null = null;

  if (!modalContainer) {
    modalContainer = document.createElement("div");
    document.body.appendChild(modalContainer);
  }

  let modalInstance = createApp({
    render() {
      const modalVNode = h(QRModal, {
        isOpen: isOpen.value,
        qrCodeUrl: qrCodeUrl.value,
        qrText: qrText.value,
        onClose: () => {
          data.onClose()
          modalInstance.unmount()
          modalContainer.innerHTML = "";
        },
        onSuccess: () => {
          data.onSuccess()
          modalInstance.unmount()
          modalContainer.innerHTML = ""
        }
      });
      return modalVNode
    }
  })

  modalInstance.mount(modalContainer)
};

const closeModal = (): void => {
  isOpen.value = false;
};

export const useQRModal = (): QRModalMethods => {
  return {
    openModal,
    closeModal,
  };
};

export const QRModalPlugin: Plugin = {
  install: (app: App): void => {
    app.provide(QRModalKey, { openModal, closeModal });
  },
};
