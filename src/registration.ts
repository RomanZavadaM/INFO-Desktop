import { invoke } from "@tauri-apps/api/core";

export interface RegistrationSummary {
  server: string;
  port: number;
  registration_id: string;
  expires_at: string;
  instance: string;
}

export async function importRegistration(text: string): Promise<RegistrationSummary> {
  return invoke<RegistrationSummary>("import_registration_text", { text });
}
