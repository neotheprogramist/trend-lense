<script lang="ts">
  interface IProps {
    showModal: boolean;
  }

  let { showModal = $bindable() }: IProps = $props();

  let dialog: HTMLDialogElement;

  $effect(() => {
    if (dialog && showModal) dialog.showModal();
    if (dialog && !showModal) dialog.close();
  });
</script>

<dialog
  bind:this={dialog}
  on:close={() => (showModal = false)}
  on:click|self={() => dialog.close()}
>
  <div on:click|stopPropagation>
    <slot name="header" />
    <hr />
    <slot />
  </div>
</dialog>

<style>
  dialog {
    max-width: 32em;
    border-radius: 1em;
    border: 1px solid rgba(128, 128, 128, 0.644);
    padding: 1em 1.5em;
    background-color: #0c0a09;
  }

  dialog::backdrop {
    background: rgba(0, 0, 0, 0.6);
  }

  dialog > div {
    padding: 1em;
  }

  dialog[open] {
    animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  @keyframes zoom {
    from {
      transform: scale(0.95);
    }
    to {
      transform: scale(1);
    }
  }

  dialog[open]::backdrop {
    animation: fade 0.2s ease-out;
  }

  @keyframes fade {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  button {
    display: block;
  }
</style>
