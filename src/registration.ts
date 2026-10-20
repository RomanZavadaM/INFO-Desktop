import { invoke } from "@tauri-apps/api/core";

export interface RegistrationSummary {
  server: string;
  port: number;
  registration_id: string;
  server_certificate_der: string;
  expires_at: string;
  instance: string;
}
export interface ActivationResult {
  device_id: string;
  credential_expires_at: string;
  storage: string;
}

export async function importRegistration(text: string): Promise<RegistrationSummary> {
  return invoke<RegistrationSummary>("import_registration_text", { text });
}
export async function activateRegistration(
  registration: RegistrationSummary,
  deviceName: string,
): Promise<ActivationResult> {
  return invoke<ActivationResult>("activate_registration", {
    server: registration.server,
    port: registration.port,
    registrationId: registration.registration_id,
    deviceName,
  });
}

export async function rotateDeviceCredential(
  server: string,
  deviceId: string,
): Promise<ActivationResult> {
  return invoke<ActivationResult>("rotate_device_credential", { server, deviceId });
}
