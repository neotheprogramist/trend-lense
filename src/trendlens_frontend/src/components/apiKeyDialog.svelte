<script lang="ts">
  import * as Dialog from "$components/shad/ui/dialog/index";
  import type { ApiRegisterStatusType } from "$lib/apiAddition";
  import { Exchanges } from "$lib/exchange";
  import type { ApiWithSecret } from "$lib/keystore.svelte";
  import BindableSelect from "./bindableSelect.svelte";
  import Button from "./shad/ui/button/button.svelte";
  import Input from "./shad/ui/input/input.svelte";
  import { Label } from "./shad/ui/label/index";

  interface IProps {
    onUpload: (localData: ApiWithSecret) => Promise<ApiRegisterStatusType>;
  }

  let { onUpload }: IProps = $props();
  let open = $state(false);
  let apiKey = $state<string>("");
  let secretKey = $state<string>("");
  let passphrase = $state<string>("");
  let exchange = $state<Exchanges | null>(null);
  let status = $state<string>("");

  function checkInputs(): boolean {
    if (!exchange) {
      status = "Exchange is required";
      return false;
    }

    if (apiKey.length == 0 || secretKey.length == 0 || passphrase.length == 0) {
      status = "Invalid keys";
      return false;
    }

    return true;
  }

  async function handleClick() {
    if (!checkInputs()) return;

    const localData: ApiWithSecret = {
      apiKey,
      secretKey,
      passphrase,
      exchange: exchange!,
    };

    status = "registering...";
    status = await onUpload(localData);

    if (status == "key registered") {
      open = false;
    }
  }
</script>

<Dialog.Root {open}>
  <Dialog.Trigger><Button>Add</Button></Dialog.Trigger>

  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Upload new key</Dialog.Title>
      <Dialog.Description
        >Enter api key, secret key and passphrase.</Dialog.Description
      >
    </Dialog.Header>

    <div class="grid gap-3 py-4">
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="api_key">Exchange</Label>
        <div class="w-100 col-span-3">
          <BindableSelect
            bind:value={exchange}
            items={Object.keys(Exchanges)}
            placeholder={"exchange"}
          />
        </div>
      </div>

      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="api_key">Api key</Label>
        <Input
          id="api_key"
          class="col-span-3"
          placeholder="enter api key"
          bind:value={apiKey}
        />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="secret_key">Secret key</Label>
        <Input
          id="secret_key"
          class="col-span-3"
          placeholder="enter secret key"
          bind:value={secretKey}
        />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="passphrase" class="text-right">Passphrase</Label>
        <Input
          id="passphrase"
          class="col-span-3"
          placeholder="enter passphrase"
          bind:value={passphrase}
        />
      </div>
    </div>
    <Dialog.Footer>
      <span class="mr-auto p-3 text-sm">{status}</span>
      <Button type="submit" on:click={handleClick}>Upload</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
