import { invoke } from "@tauri-apps/api/core";

export interface RegistrationSummary {
  server: string;
  transport: string;
  port: number;
  server_public_key: string;
  bootstrap_address: string;
  private_server: string;
  registration_id: string;
  expires_at: string;
  instance: string;
}

export interface DevicePublicIdentity {
  registration_id: string;
  public_key: string;
  created: boolean;
  storage: string;
}

export async function importRegistration(text: string): Promise<RegistrationSummary> {
  return invoke<RegistrationSummary>("import_registration_text", { text });
}

export async function ensureDeviceKey(registrationId: string): Promise<DevicePublicIdentity> {
  return invoke<DevicePublicIdentity>("ensure_device_key", { registrationId });
}
