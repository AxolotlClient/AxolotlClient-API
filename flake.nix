{
	inputs.nixpkgs.url = "nixpkgs/nixos-unstable";

	outputs = { self, nixpkgs }: let
		version = "${self.shortRev or "dirty"}";
	in {
		packages = nixpkgs.lib.genAttrs [ "aarch64-linux" "x86_64-linux" ] (system: let
			pkgs = import nixpkgs { inherit system; };
		in {
			default = pkgs.callPackage (
				{ rustPlatform, lib }: rustPlatform.buildRustPackage rec {
					pname = "axolotl_client-api";
					version = "0.0.0";

					src = builtins.path {
						name = "axolotl_client-api-${version}";
						path = lib.cleanSource ./.;
					};

					nativeBuildInputs = [ pkgs.pkg-config ];
					buildInputs = [ pkgs.openssl ];

					SQLX_OFFLINE = true;

					cargoLock.lockFile = ./Cargo.lock;
				}
			) {};
		});

		nixosModules.default = { config, lib, pkgs, ... }: with lib; let cfg = config.services.axolotlClientApi; in {
			options.services.axolotlClientApi = {
				enable = mkEnableOption "AxolotlClient-API";

				postgresUrl = mkOption {
					type = types.nullOr types.str;
					description = "Postgres Connection Url, see: <https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html>. Mutually exclusive with postgresUrlFile.";
					default = null;
				};

				postgresUrlFile = mkOption {
					type = types.nullOr types.path;
					description = "File containing a Postgres Connection Url, see: <https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html>. Mutually exclusive with postgresUrl.";
					default = null;
				};

				hypixelApiKey = mkOption {
					type = types.nullOr types.str;
					description = "Hypixel API Key. Mutually exclusive with hypixelApiKeyFile.";
					default = null;
				};

				hypixelApiKeyFile = mkOption {
					type = types.nullOr types.path;
					description = "File containing a Hypixel API Key. Mutually exclusive with hypixelApiKey.";
					default = null;
				};

				notesFile = mkOption {
					type = types.nullOr types.path;
					description = "File containing notes to be returned by the Api.";
					default = null;
				};
			};

			config = mkIf cfg.enable {
				users.users.axolotl_client-api = { isSystemUser = true; name = "axolotl_client-api"; group = "axolotl_client-api"; };
				users.groups.axolotl_client-api = {};

				systemd.services.axolotl_client-api = {
					description = "AxolotlClient API Service";

					after = [ "postgresql.service" ];
					requires = [ "postgresql.service" ];

					upheldBy = [ "multi-user.target" ];

					serviceConfig = with config.age.secrets; {
						User = "axolotl_client-api";
						Group = "axolotl_client-api";

						Type = "exec";
						
						# Would be nice if we validated this to ensure that we aren't passing a set of invalid options, but oh well.
						ExecStart = ''
							${self.packages.${pkgs.stdenv.hostPlatform.system}.default}/bin/axolotl_client-api \
								${optionalString (cfg.postgresUrl != null) "--postgres-url ${cfg.postgresUrl}"} \
								${optionalString (cfg.postgresUrlFile != null) "--postgres-url-file ${cfg.postgresUrlFile}"} \
								${optionalString (cfg.hypixelApiKey != null) "--hypixel-api-key ${cfg.hypixelApiKey}"} \
								${optionalString (cfg.hypixelApiKeyFile != null) "--hypixel-api-key-file ${cfg.hypixelApiKeyFile}"} \
								${optionalString (cfg.notesFile != null) "--notes-file ${cfg.notesFile}"}
						'';

						environment.RUST_BACKTRACE = true;

						# Why can't this shit just be the default?
						CapabilityBoundingSet = "";
						LockPersonality = true;
						MemoryDenyWriteExecute = true;
						NoNewPrivileges = true;
						PrivateDevices = true;
						PrivateMounts = true;
						PrivateTmp = true;
						PrivateUsers = true;
						ProcSubset = "pid";
						ProtectClock = true;
						ProtectControlGroups = true;
						ProtectHome = true;
						ProtectHostname = true;
						ProtectKernelLogs = true;
						ProtectKernelModules = true;
						ProtectKernelTunables = true;
						ProtectProc = "invisible";
						ProtectSystem = "strict";
						RemoveIPC = true;
						RestrictAddressFamilies = "AF_UNIX AF_INET AF_INET6";
						RestrictNamespaces = true;
						RestrictRealtime = true;
						RestrictSUIDSGID = true;
						SystemCallArchitectures = "native";
						SystemCallFilter = "@basic-io @file-system @io-event @network-io @process @signal ioctl madvise";
						UMask = "777";
					};
				};
			};
		};
	};
}
