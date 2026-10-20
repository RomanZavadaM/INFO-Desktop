import { invoke } from "@tauri-apps/api/core";

export interface RegistrationFile {
  server: string;
  transport: string;
  port: number;
  server_public_key: string;
  registration_id: string;
  registration_token: string;
  expires_at: string;
  instance: string;
}

export async function parseRegistration(text: string): Promise<RegistrationFile> {
  return invoke<RegistrationFile>("parse_registration_text", { text });
}
