import {
  load as tauriLoad,
  Store as TauriStore,
} from "@tauri-apps/plugin-store";

type Stored = {
  isEditMode: boolean;
  selectedAwsProfile: string;
};

type StoredKey = keyof Stored;

class Store {
  private readonly DEFAULT_FILENAME = ".settings.dat";

  private filename: string;

  private store?: TauriStore;

  private async load() {
    const result = await tauriLoad(this.filename);

    this.store = result;
  }

  private async prepareStore() {
    if (this.store === undefined) {
      await this.load();
    }

    if (this.store === undefined) {
      throw new Error("Unable to locate store");
    }

    return this.store;
  }

  constructor(filename = this.DEFAULT_FILENAME) {
    this.filename = filename;
  }

  async getItem<T extends StoredKey>(key: T): Promise<Stored[T] | undefined> {
    await this.prepareStore();

    const result = await this.store?.get<Stored[T]>(key);

    return result;
  }

  async save() {
    await this.prepareStore();

    return this.store?.save();
  }

  async setItem<T extends StoredKey>(key: StoredKey, value: Stored[T]) {
    await this.prepareStore();

    await this.store?.set(key, value);
  }
}

export const AppStore = new Store();
