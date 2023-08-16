// Copyright 2023 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// DevContainer Definition schema
// {
// 	"$schema": "https://json-schema.org/draft/2019-09/schema",
// 	"description": "Defines a dev container",
// 	"allowComments": true,
// 	"allowTrailingCommas": false,
// 	"definitions": {
// 		"devContainerCommon": {
// 			"type": "object",
// 			"properties": {
// 				"$schema": {
// 					"type": "string",
// 					"format": "uri",
// 					"description": "The JSON schema of the `devcontainer.json` file."
// 				},
// 				"name": {
// 					"type": "string",
// 					"description": "A name for the dev container which can be displayed to the user."
// 				},
// 				"features": {
// 					"type": "object",
// 					"description": "Features to add to the dev container.",
// 					"properties": {
// 						"fish": {
// 							"deprecated": true,
// 							"deprecationMessage": "Legacy feature will be removed in the future. Please check https://containers.dev/features for replacements."
// 						},
// 						"maven": {
// 							"deprecated": true,
// 							"deprecationMessage": "Legacy feature will be removed in the future. Please check https://containers.dev/features for replacements. E.g., `ghcr.io/devcontainers/features/java` has an option to install Maven."
// 						},
// 						"gradle": {
// 							"deprecated": true,
// 							"deprecationMessage": "Legacy feature will be removed in the future. Please check https://containers.dev/features for replacements. E.g., `ghcr.io/devcontainers/features/java` has an option to install Gradle."
// 						},
// 						"homebrew": {
// 							"deprecated": true,
// 							"deprecationMessage": "Legacy feature will be removed in the future. Please check https://containers.dev/features for replacements."
// 						},
// 						"jupyterlab": {
// 							"deprecated": true,
// 							"deprecationMessage": "Legacy feature will be removed in the future. Please check https://containers.dev/features for replacements. E.g., `ghcr.io/devcontainers/features/python` has an option to install JupyterLab."
// 						}
// 					},
// 					"additionalProperties": true
// 				},
// 				"overrideFeatureInstallOrder": {
// 					"type": "array",
// 					"description": "Array consisting of the Feature id (without the semantic version) of Features in the order the user wants them to be installed.",
// 					"items": {
// 						"type": "string"
// 					}
// 				},
// 				"forwardPorts": {
// 					"type": "array",
// 					"description": "Ports that are forwarded from the container to the local machine. Can be an integer port number, or a string of the format \"host:port_number\".",
// 					"items": {
// 						"oneOf": [
// 							{
// 								"type": "integer",
// 								"maximum": 65535,
// 								"minimum": 0
// 							},
// 							{
// 								"type": "string",
// 								"pattern": "^([a-z0-9-]+):(\\d{1,5})$"
// 							}
// 						]
// 					}
// 				},
// 				"portsAttributes": {
// 					"type": "object",
// 					"patternProperties": {
// 						"(^\\d+(-\\d+)?$)|(.+)": {
// 							"type": "object",
// 							"description": "A port, range of ports (ex. \"40000-55000\"), or regular expression (ex. \".+\\\\/server.js\").  For a port number or range, the attributes will apply to that port number or range of port numbers. Attributes which use a regular expression will apply to ports whose associated process command line matches the expression.",
// 							"properties": {
// 								"onAutoForward": {
// 									"type": "string",
// 									"enum": [
// 										"notify",
// 										"openBrowser",
// 										"openBrowserOnce",
// 										"openPreview",
// 										"silent",
// 										"ignore"
// 									],
// 									"enumDescriptions": [
// 										"Shows a notification when a port is automatically forwarded.",
// 										"Opens the browser when the port is automatically forwarded. Depending on your settings, this could open an embedded browser.",
// 										"Opens the browser when the port is automatically forwarded, but only the first time the port is forward during a session. Depending on your settings, this could open an embedded browser.",
// 										"Opens a preview in the same window when the port is automatically forwarded.",
// 										"Shows no notification and takes no action when this port is automatically forwarded.",
// 										"This port will not be automatically forwarded."
// 									],
// 									"description": "Defines the action that occurs when the port is discovered for automatic forwarding",
// 									"default": "notify"
// 								},
// 								"elevateIfNeeded": {
// 									"type": "boolean",
// 									"description": "Automatically prompt for elevation (if needed) when this port is forwarded. Elevate is required if the local port is a privileged port.",
// 									"default": false
// 								},
// 								"label": {
// 									"type": "string",
// 									"description": "Label that will be shown in the UI for this port.",
// 									"default": "Application"
// 								},
// 								"requireLocalPort": {
// 									"type": "boolean",
// 									"markdownDescription": "When true, a modal dialog will show if the chosen local port isn't used for forwarding.",
// 									"default": false
// 								},
// 								"protocol": {
// 									"type": "string",
// 									"enum": [
// 										"http",
// 										"https"
// 									],
// 									"description": "The protocol to use when forwarding this port."
// 								}
// 							},
// 							"default": {
// 								"label": "Application",
// 								"onAutoForward": "notify"
// 							}
// 						}
// 					},
// 					"markdownDescription": "Set default properties that are applied when a specific port number is forwarded. For example:\n\n```\n\"3000\": {\n  \"label\": \"Application\"\n},\n\"40000-55000\": {\n  \"onAutoForward\": \"ignore\"\n},\n\".+\\\\/server.js\": {\n \"onAutoForward\": \"openPreview\"\n}\n```",
// 					"defaultSnippets": [
// 						{
// 							"body": {
// 								"${1:3000}": {
// 									"label": "${2:Application}",
// 									"onAutoForward": "notify"
// 								}
// 							}
// 						}
// 					],
// 					"additionalProperties": false
// 				},
// 				"otherPortsAttributes": {
// 					"type": "object",
// 					"properties": {
// 						"onAutoForward": {
// 							"type": "string",
// 							"enum": [
// 								"notify",
// 								"openBrowser",
// 								"openPreview",
// 								"silent",
// 								"ignore"
// 							],
// 							"enumDescriptions": [
// 								"Shows a notification when a port is automatically forwarded.",
// 								"Opens the browser when the port is automatically forwarded. Depending on your settings, this could open an embedded browser.",
// 								"Opens a preview in the same window when the port is automatically forwarded.",
// 								"Shows no notification and takes no action when this port is automatically forwarded.",
// 								"This port will not be automatically forwarded."
// 							],
// 							"description": "Defines the action that occurs when the port is discovered for automatic forwarding",
// 							"default": "notify"
// 						},
// 						"elevateIfNeeded": {
// 							"type": "boolean",
// 							"description": "Automatically prompt for elevation (if needed) when this port is forwarded. Elevate is required if the local port is a privileged port.",
// 							"default": false
// 						},
// 						"label": {
// 							"type": "string",
// 							"description": "Label that will be shown in the UI for this port.",
// 							"default": "Application"
// 						},
// 						"requireLocalPort": {
// 							"type": "boolean",
// 							"markdownDescription": "When true, a modal dialog will show if the chosen local port isn't used for forwarding.",
// 							"default": false
// 						},
// 						"protocol": {
// 							"type": "string",
// 							"enum": [
// 								"http",
// 								"https"
// 							],
// 							"description": "The protocol to use when forwarding this port."
// 						}
// 					},
// 					"defaultSnippets": [
// 						{
// 							"body": {
// 								"onAutoForward": "ignore"
// 							}
// 						}
// 					],
// 					"markdownDescription": "Set default properties that are applied to all ports that don't get properties from the setting `remote.portsAttributes`. For example:\n\n```\n{\n  \"onAutoForward\": \"ignore\"\n}\n```",
// 					"additionalProperties": false
// 				},
// 				"updateRemoteUserUID": {
// 					"type": "boolean",
// 					"description": "Controls whether on Linux the container's user should be updated with the local user's UID and GID. On by default when opening from a local folder."
// 				},
// 				"containerEnv": {
// 					"type": "object",
// 					"additionalProperties": {
// 						"type": "string"
// 					},
// 					"description": "Container environment variables."
// 				},
// 				"containerUser": {
// 					"type": "string",
// 					"description": "The user the container will be started with. The default is the user on the Docker image."
// 				},
// 				"mounts": {
// 					"type": "array",
// 					"description": "Mount points to set up when creating the container. See Docker's documentation for the --mount option for the supported syntax.",
// 					"items": {
// 						"anyOf": [
// 							{
// 								"$ref": "#/definitions/Mount"
// 							},
// 							{
// 								"type": "string"
// 							}
// 						]
// 					}
// 				},
// 				"init": {
// 					"type": "boolean",
// 					"description": "Passes the --init flag when creating the dev container."
// 				},
// 				"privileged": {
// 					"type": "boolean",
// 					"description": "Passes the --privileged flag when creating the dev container."
// 				},
// 				"capAdd": {
// 					"type": "array",
// 					"description": "Passes docker capabilities to include when creating the dev container.",
// 					"examples": [
// 						"SYS_PTRACE"
// 					],
// 					"items": {
// 						"type": "string"
// 					}
// 				},
// 				"securityOpt": {
// 					"type": "array",
// 					"description": "Passes docker security options to include when creating the dev container.",
// 					"examples": [
// 						"seccomp=unconfined"
// 					],
// 					"items": {
// 						"type": "string"
// 					}
// 				},
// 				"remoteEnv": {
// 					"type": "object",
// 					"additionalProperties": {
// 						"type": [
// 							"string",
// 							"null"
// 						]
// 					},
// 					"description": "Remote environment variables to set for processes spawned in the container including lifecycle scripts and any remote editor/IDE server process."
// 				},
// 				"remoteUser": {
// 					"type": "string",
// 					"description": "The username to use for spawning processes in the container including lifecycle scripts and any remote editor/IDE server process. The default is the same user as the container."
// 				},
// 				"initializeCommand": {
// 					"type": [
// 						"string",
// 						"array",
// 						"object"
// 					],
// 					"description": "A command to run locally (i.e Your host machine, cloud VM) before anything else. This command is run before \"onCreateCommand\". If this is a single string, it will be run in a shell. If this is an array of strings, it will be run as a single command without shell. If this is an object, each provided command will be run in parallel.",
// 					"items": {
// 						"type": "string"
// 					},
// 					"additionalProperties": {
// 						"type": [
// 							"string",
// 							"array"
// 						],
// 						"items": {
// 							"type": "string"
// 						}
// 					}
// 				},
// 				"onCreateCommand": {
// 					"type": [
// 						"string",
// 						"array",
// 						"object"
// 					],
// 					"description": "A command to run when creating the container. This command is run after \"initializeCommand\" and before \"updateContentCommand\". If this is a single string, it will be run in a shell. If this is an array of strings, it will be run as a single command without shell. If this is an object, each provided command will be run in parallel.",
// 					"items": {
// 						"type": "string"
// 					},
// 					"additionalProperties": {
// 						"type": [
// 							"string",
// 							"array"
// 						],
// 						"items": {
// 							"type": "string"
// 						}
// 					}
// 				},
// 				"updateContentCommand": {
// 					"type": [
// 						"string",
// 						"array",
// 						"object"
// 					],
// 					"description": "A command to run when creating the container and rerun when the workspace content was updated while creating the container. This command is run after \"onCreateCommand\" and before \"postCreateCommand\". If this is a single string, it will be run in a shell. If this is an array of strings, it will be run as a single command without shell. If this is an object, each provided command will be run in parallel.",
// 					"items": {
// 						"type": "string"
// 					},
// 					"additionalProperties": {
// 						"type": [
// 							"string",
// 							"array"
// 						],
// 						"items": {
// 							"type": "string"
// 						}
// 					}
// 				},
// 				"postCreateCommand": {
// 					"type": [
// 						"string",
// 						"array",
// 						"object"
// 					],
// 					"description": "A command to run after creating the container. This command is run after \"updateContentCommand\" and before \"postStartCommand\". If this is a single string, it will be run in a shell. If this is an array of strings, it will be run as a single command without shell. If this is an object, each provided command will be run in parallel.",
// 					"items": {
// 						"type": "string"
// 					},
// 					"additionalProperties": {
// 						"type": [
// 							"string",
// 							"array"
// 						],
// 						"items": {
// 							"type": "string"
// 						}
// 					}
// 				},
// 				"postStartCommand": {
// 					"type": [
// 						"string",
// 						"array",
// 						"object"
// 					],
// 					"description": "A command to run after starting the container. This command is run after \"postCreateCommand\" and before \"postAttachCommand\". If this is a single string, it will be run in a shell. If this is an array of strings, it will be run as a single command without shell. If this is an object, each provided command will be run in parallel.",
// 					"items": {
// 						"type": "string"
// 					},
// 					"additionalProperties": {
// 						"type": [
// 							"string",
// 							"array"
// 						],
// 						"items": {
// 							"type": "string"
// 						}
// 					}
// 				},
// 				"postAttachCommand": {
// 					"type": [
// 						"string",
// 						"array",
// 						"object"
// 					],
// 					"description": "A command to run when attaching to the container. This command is run after \"postStartCommand\". If this is a single string, it will be run in a shell. If this is an array of strings, it will be run as a single command without shell. If this is an object, each provided command will be run in parallel.",
// 					"items": {
// 						"type": "string"
// 					},
// 					"additionalProperties": {
// 						"type": [
// 							"string",
// 							"array"
// 						],
// 						"items": {
// 							"type": "string"
// 						}
// 					}
// 				},
// 				"waitFor": {
// 					"type": "string",
// 					"enum": [
// 						"initializeCommand",
// 						"onCreateCommand",
// 						"updateContentCommand",
// 						"postCreateCommand",
// 						"postStartCommand"
// 					],
// 					"description": "The user command to wait for before continuing execution in the background while the UI is starting up. The default is \"updateContentCommand\"."
// 				},
// 				"userEnvProbe": {
// 					"type": "string",
// 					"enum": [
// 						"none",
// 						"loginShell",
// 						"loginInteractiveShell",
// 						"interactiveShell"
// 					],
// 					"description": "User environment probe to run. The default is \"loginInteractiveShell\"."
// 				},
// 				"hostRequirements": {
// 					"type": "object",
// 					"description": "Host hardware requirements.",
// 					"properties": {
// 						"cpus": {
// 							"type": "integer",
// 							"minimum": 1,
// 							"description": "Number of required CPUs."
// 						},
// 						"memory": {
// 							"type": "string",
// 							"pattern": "^\\d+([tgmk]b)?$",
// 							"description": "Amount of required RAM in bytes. Supports units tb, gb, mb and kb."
// 						},
// 						"storage": {
// 							"type": "string",
// 							"pattern": "^\\d+([tgmk]b)?$",
// 							"description": "Amount of required disk space in bytes. Supports units tb, gb, mb and kb."
// 						},
// 						"gpu": {
// 							"oneOf": [
// 								{
// 									"type": [
// 										"boolean",
// 										"string"
// 									],
// 									"enum": [
// 										true,
// 										false,
// 										"optional"
// 									],
// 									"description": "Indicates whether a GPU is required. The string \"optional\" indicates that a GPU is optional. An object value can be used to configure more detailed requirements."
// 								},
// 								{
// 									"type": "object",
// 									"properties": {
// 										"cores": {
// 											"type": "integer",
// 											"minimum": 1,
// 											"description": "Number of required cores."
// 										},
// 										"memory": {
// 											"type": "string",
// 											"pattern": "^\\d+([tgmk]b)?$",
// 											"description": "Amount of required RAM in bytes. Supports units tb, gb, mb and kb."
// 										}
// 									},
// 									"description": "Indicates whether a GPU is required. The string \"optional\" indicates that a GPU is optional. An object value can be used to configure more detailed requirements.",
// 									"additionalProperties": false
// 								}
// 							]
// 						}
// 					},
// 					"unevaluatedProperties": false
// 				},
// 				"customizations": {
// 					"type": "object",
// 					"description": "Tool-specific configuration. Each tool should use a JSON object subproperty with a unique name to group its customizations."
// 				},
// 				"additionalProperties": {
// 					"type": "object",
// 					"additionalProperties": true
// 				}
// 			}
// 		},
// 		"nonComposeBase": {
// 			"type": "object",
// 			"properties": {
// 				"appPort": {
// 					"type": [
// 						"integer",
// 						"string",
// 						"array"
// 					],
// 					"description": "Application ports that are exposed by the container. This can be a single port or an array of ports. Each port can be a number or a string. A number is mapped to the same port on the host. A string is passed to Docker unchanged and can be used to map ports differently, e.g. \"8000:8010\".",
// 					"items": {
// 						"type": [
// 							"integer",
// 							"string"
// 						]
// 					}
// 				},
// 				"runArgs": {
// 					"type": "array",
// 					"description": "The arguments required when starting in the container.",
// 					"items": {
// 						"type": "string"
// 					}
// 				},
// 				"shutdownAction": {
// 					"type": "string",
// 					"enum": [
// 						"none",
// 						"stopContainer"
// 					],
// 					"description": "Action to take when the user disconnects from the container in their editor. The default is to stop the container."
// 				},
// 				"overrideCommand": {
// 					"type": "boolean",
// 					"description": "Whether to overwrite the command specified in the image. The default is true."
// 				},
// 				"workspaceFolder": {
// 					"type": "string",
// 					"description": "The path of the workspace folder inside the container."
// 				},
// 				"workspaceMount": {
// 					"type": "string",
// 					"description": "The --mount parameter for docker run. The default is to mount the project folder at /workspaces/$project."
// 				}
// 			}
// 		},
// 		"dockerfileContainer": {
// 			"oneOf": [
// 				{
// 					"type": "object",
// 					"properties": {
// 						"build": {
// 							"type": "object",
// 							"description": "Docker build-related options.",
// 							"allOf": [
// 								{
// 									"type": "object",
// 									"properties": {
// 										"dockerfile": {
// 											"type": "string",
// 											"description": "The location of the Dockerfile that defines the contents of the container. The path is relative to the folder containing the `devcontainer.json` file."
// 										},
// 										"context": {
// 											"type": "string",
// 											"description": "The location of the context folder for building the Docker image. The path is relative to the folder containing the `devcontainer.json` file."
// 										}
// 									},
// 									"required": [
// 										"dockerfile"
// 									]
// 								},
// 								{
// 									"$ref": "#/definitions/buildOptions"
// 								}
// 							],
// 							"unevaluatedProperties": false
// 						}
// 					},
// 					"required": [
// 						"build"
// 					]
// 				},
// 				{
// 					"allOf": [
// 						{
// 							"type": "object",
// 							"properties": {
// 								"dockerFile": {
// 									"type": "string",
// 									"description": "The location of the Dockerfile that defines the contents of the container. The path is relative to the folder containing the `devcontainer.json` file."
// 								},
// 								"context": {
// 									"type": "string",
// 									"description": "The location of the context folder for building the Docker image. The path is relative to the folder containing the `devcontainer.json` file."
// 								}
// 							},
// 							"required": [
// 								"dockerFile"
// 							]
// 						},
// 						{
// 							"type": "object",
// 							"properties": {
// 								"build": {
// 									"description": "Docker build-related options.",
// 									"$ref": "#/definitions/buildOptions"
// 								}
// 							}
// 						}
// 					]
// 				}
// 			]
// 		},
// 		"buildOptions": {
// 			"type": "object",
// 			"properties": {
// 				"target": {
// 					"type": "string",
// 					"description": "Target stage in a multi-stage build."
// 				},
// 				"args": {
// 					"type": "object",
// 					"additionalProperties": {
// 						"type": [
// 							"string"
// 						]
// 					},
// 					"description": "Build arguments."
// 				},
// 				"cacheFrom": {
// 					"type": [
// 						"string",
// 						"array"
// 					],
// 					"description": "The image to consider as a cache. Use an array to specify multiple images.",
// 					"items": {
// 						"type": "string"
// 					}
// 				}
// 			}
// 		},
// 		"imageContainer": {
// 			"type": "object",
// 			"properties": {
// 				"image": {
// 					"type": "string",
// 					"description": "The docker image that will be used to create the container."
// 				}
// 			},
// 			"required": [
// 				"image"
// 			]
// 		},
// 		"composeContainer": {
// 			"type": "object",
// 			"properties": {
// 				"dockerComposeFile": {
// 					"type": [
// 						"string",
// 						"array"
// 					],
// 					"description": "The name of the docker-compose file(s) used to start the services.",
// 					"items": {
// 						"type": "string"
// 					}
// 				},
// 				"service": {
// 					"type": "string",
// 					"description": "The service you want to work on. This is considered the primary container for your dev environment which your editor will connect to."
// 				},
// 				"runServices": {
// 					"type": "array",
// 					"description": "An array of services that should be started and stopped.",
// 					"items": {
// 						"type": "string"
// 					}
// 				},
// 				"workspaceFolder": {
// 					"type": "string",
// 					"description": "The path of the workspace folder inside the container. This is typically the target path of a volume mount in the docker-compose.yml."
// 				},
// 				"shutdownAction": {
// 					"type": "string",
// 					"enum": [
// 						"none",
// 						"stopCompose"
// 					],
// 					"description": "Action to take when the user disconnects from the primary container in their editor. The default is to stop all of the compose containers."
// 				},
// 				"overrideCommand": {
// 					"type": "boolean",
// 					"description": "Whether to overwrite the command specified in the image. The default is false."
// 				}
// 			},
// 			"required": [
// 				"dockerComposeFile",
// 				"service",
// 				"workspaceFolder"
// 			]
// 		},
// 		"Mount": {
// 			"type": "object",
// 			"properties": {
// 				"type": {
// 					"type": "string",
// 					"enum": [
// 						"bind",
// 						"volume"
// 					],
// 					"description": "Mount type."
// 				},
// 				"source": {
// 					"type": "string",
// 					"description": "Mount source."
// 				},
// 				"target": {
// 					"type": "string",
// 					"description": "Mount target."
// 				}
// 			},
// 			"required": [
// 				"type",
// 				"source",
// 				"target"
// 			],
// 			"additionalProperties": false
// 		}
// 	},
// 	"oneOf": [
// 		{
// 			"allOf": [
// 				{
// 					"oneOf": [
// 						{
// 							"allOf": [
// 								{
// 									"oneOf": [
// 										{
// 											"$ref": "#/definitions/dockerfileContainer"
// 										},
// 										{
// 											"$ref": "#/definitions/imageContainer"
// 										}
// 									]
// 								},
// 								{
// 									"$ref": "#/definitions/nonComposeBase"
// 								}
// 							]
// 						},
// 						{
// 							"$ref": "#/definitions/composeContainer"
// 						}
// 					]
// 				},
// 				{
// 					"$ref": "#/definitions/devContainerCommon"
// 				}
// 			]
// 		},
// 		{
// 			"type": "object",
// 			"$ref": "#/definitions/devContainerCommon",
// 			"additionalProperties": false
// 		}
// 	],
// 	"unevaluatedProperties": false
// }

use std::collections::HashMap;

use serde_json::Value;

pub struct DevContainerCommon {
    /// The name for the dev container which can be displayed to the user.
    pub name: Option<String>,
    /// Features to add to the dev container.
    pub features: Option<HashMap<String, String>>,
    /// ray consisting of the Feature id (without the semantic version) of Features
    /// in the order the user wants them to be installed.
    pub override_feature_install_order: Option<Vec<String>>,
    /// Ports that are forwarded from the container to the local machine.
    /// Can be an integer port number, or a string of the format \"host:port_number\"."
    pub forward_ports: Option<Vec<IntegerOrString>>,
    /// Object that maps a port number, "host:port" value, range, or regular expression to a set of default options.
    pub ports_attributes: Option<HashMap<String, PortAttributes>>,
    /// Default options for ports, port ranges, and hosts that aren’t configured using `PortsAttributes`.
    pub other_ports_attributes: Option<PortAttributes>,
    /// Controls whether on Linux the container's user should be updated with the local user's UID and GID.
    /// On by default when opening from a local folder.
    pub update_remote_user_uid: Option<bool>,
    /// Container environment variables.
    pub container_env: Option<HashMap<String, String>>,
    /// The user the container will be started with. The default is the user on the Docker image.
    pub container_user: Option<String>,
    /// ount points to set up when creating the container.
    ///  See Docker's documentation for the --mount option for the supported syntax.
    pub mounts: Option<Vec<StringOrMount>>,
    /// Passes the --init flag when creating the dev container.
    pub init: Option<bool>,
    /// Passes the --privileged flag when creating the dev container.
    pub privileged: Option<bool>,
    /// Passes docker capabilities to include when creating the dev container.
    pub cap_add: Option<Vec<String>>,
    /// Passes docker security options to include when creating the dev container.
    pub security_opt: Option<Vec<String>>,
    /// Remote environment variables to set for processes spawned in the container
    /// including lifecycle scripts and any remote editor/IDE server process.
    pub remote_env: Option<HashMap<String, String>>,
    /// The username to use for spawning processes in the container
    /// including lifecycle scripts and any remote editor/IDE server process.
    /// The default is the same user as the container.
    pub remote_user: Option<String>,
    /// When creating or working with a dev container, you may need different
    /// commands to be run at different points in the container’s lifecycle.
    pub lifecycle_scripts: Option<LifecycleScripts>,
    /// User environment probe to run. The default is "loginInteractiveShell"
    pub user_env_probe: Option<UserEnvProbe>,
    /// Host hardware requirements.
    pub host_requirements: Option<HostRequirements>,
    pub customizations: Option<HashMap<String, Value>>,
    pub additional_properties: Option<HashMap<String, Value>>,
}

pub enum IntegerOrString {
    /// minimum: 0, maximum: 65535
    Integer(i32),
    /// pattern: ^([a-z0-9-]+):(\\d{1,5})$
    String(String),
}

/// The PortAttributes properties allow you to map default port options
/// for one or more manually or automatically forwarded ports.
pub struct PortAttributes {
    /// Defines the action that occurs when the port is discovered for automatic forwarding
    pub on_auto_forward: Option<OnAutoForward>,
    /// Automatically prompt for elevation (if needed) when this port is forwarded.
    /// Elevate is required if the local port is a privileged port.
    pub elevate_if_needed: Option<bool>,
    /// Label that will be shown in the UI for this port.
    // TODO: default: Application
    pub label: Option<String>,
    /// When true, a modal dialog will show if the chosen local port isn't used for forwarding.
    pub require_local_port: Option<bool>,
    /// The protocol to use when forwarding this port.
    pub protocol: Option<Protocol>,
}

impl Default for PortAttributes {
    fn default() -> Self {
        Self {
            on_auto_forward: Some(OnAutoForward::Notify),
            elevate_if_needed: Some(false),
            label: Some("Application".to_string()),
            require_local_port: Some(false),
            protocol: None,
        }
    }
}

/// Defines the action that occurs when the port is discovered for automatic forwarding
pub enum OnAutoForward {
    /// Shows a notification when a port is automatically forwarded.
    // TODO: default
    Notify,
    /// Opens the browser when the port is automatically forwarded.
    /// Depending on your settings, this could open an embedded browser.
    OpenBrowser,
    /// Opens the browser when the port is automatically forwarded,
    /// but only the first time the port is forward during a session.
    /// Depending on your settings, this could open an embedded browser.
    OpenBrowserOnce,
    /// Opens a preview in the same window when the port is automatically forwarded.
    OpenPreview,
    /// Shows no notification and takes no action when this port is automatically forwarded.
    Silent,
    /// This port will not be automatically forwarded.
    Ignore,
}

/// The Protocol to use when forwarding a port.
pub enum Protocol {
    Http,
    Https,
}

/// A mount can be a bind mount or a volume mount.
pub enum StringOrMount {
    String(String),
    Mount(Mount),
}

pub struct Mount {
    /// Mount type.
    pub kind: MountKind,
    /// Mount source.
    pub source: String,
    /// Mount target.
    pub target: String,
}

/// Mount type.
pub enum MountKind {
    Bind,
    Volume,
}

/// When creating or working with a dev container, you may need different
/// commands to be run at different points in the container’s lifecycle.
pub struct LifecycleScripts {
    /// A command to run locally (i.e Your host machine, cloud VM) before anything else.
    /// This command is run before "onCreateCommand"". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.
    pub initialize_command: Option<LifecycleScript>,
    /// A command to run when creating the container.
    /// This command is run after "initializeCommand" and before "updateContentCommand".
    /// If this is a single string, it will be run in a shell. If this is an array of strings,
    /// it will be run as a single command without shell. If this is an object,
    /// each provided command will be run in parallel.
    pub on_create_command: Option<LifecycleScript>,
    /// A command to run when creating the container and rerun when the workspace content
    /// was updated while creating the container. This command is run after "onCreateCommand"
    /// and before "postCreateCommand". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.
    pub update_content_command: Option<LifecycleScript>,
    /// A command to run after creating the container. This command is run after "updateContentCommand"
    /// and before "postStartCommand". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.
    pub post_create_command: Option<LifecycleScript>,
    /// A command to run after starting the container. This command is run after "postCreateCommand"
    /// and before "postAttachCommand". If this is a single string, it will be run in a shell.
    /// If this is an array of strings, it will be run as a single command without shell.
    /// If this is an object, each provided command will be run in parallel.",
    pub post_start_command: Option<LifecycleScript>,
    /// A command to run when attaching to the container. This command is run after "postStartCommand".
    /// If this is a single string, it will be run in a shell. If this is an array of strings,
    /// it will be run as a single command without shell. If this is an object,
    /// each provided command will be run in parallel.
    pub post_attach_command: Option<LifecycleScript>,
    /// The user command to wait for before continuing execution in the background
    /// while the UI is starting up. The default is "updateContentCommand"
    pub wait_for: Option<WaitFor>,
}

pub enum LifecycleScript {
    String(String),
    Array(Vec<String>),
    Object(HashMap<String, String>),
}

pub enum WaitFor {
    InitializeCommand,
    OnCreateCommand,
    UpdateContentCommand,
    PostCreateCommand,
    PostStartCommand,
}

pub enum UserEnvProbe {
    None,
    LoginShell,
    // TODO: default
    LoginInteractiveShell,
    InteractiveShell,
}

/// Host hardware requirements.
pub struct HostRequirements {
    /// Number of required CPUs. minimum: 1
    pub cpus: Option<u32>,
    /// Amount of required RAM in bytes. Supports units tb, gb, mb and kb.
    /// pattern: ^\\d+([tgmk]b)?$
    pub memory: Option<String>,
    /// Amount of required disk space in bytes. Supports units tb, gb, mb and kb.
    /// pattern: ^\\d+([tgmk]b)?$
    pub storage: Option<String>,
    /// Indicates whether a GPU is required.
    /// The string "optional" indicates that a GPU is optional.
    /// An object value can be used to configure more detailed requirements.
    pub gpu: Option<GPUVar>,
}

pub enum GPUVar {
    /// Indicates whether a GPU is required.
    Boolean(bool),
    /// The string "optional" indicates that a GPU is optional.
    String(String),
    /// An object value can be used to configure more detailed requirements.
    Config(GPUConfig),
}

pub struct GPUConfig {
    /// Number of required cores. minimum: 1
    pub cores: Option<u32>,
    /// Amount of required RAM in bytes. Supports units tb, gb, mb and kb.
    /// pattern: ^\\d+([tgmk]b)?$
    pub memory: Option<String>,
}

pub struct NonComposeBase {
    /// Application ports that are exposed by the container.
    /// This can be a single port or an array of ports. Each port can be a number or a string.
    /// A number is mapped to the same port on the host. A string is passed to Docker unchanged
    /// and can be used to map ports differently, e.g. "8000:8010".
    pub app_port: Option<IntegerOrStringOrArray>,
    /// The arguments required when starting in the container.
    pub run_args: Option<Vec<String>>,
    /// Action to take when the user disconnects from the container in their editor.
    /// The default is to stop the container.
    pub shutdown_action: Option<ShutdownAction>,
    /// Whether to overwrite the command specified in the image. The default is true.
    pub override_command: Option<bool>,
    /// The path of the workspace folder inside the container.
    pub workspace_folder: Option<String>,
    /// The --mount parameter for docker run. The default is to mount the project folder at /workspaces/$project.
    pub workspace_mount: Option<String>,
}

pub enum IntegerOrStringOrArray {
    Integer(u32),
    String(String),
    Array(Vec<IntegerOrString>),
}

pub enum ShutdownAction {
    None,
    StopContainer,
    StopCompose,
}

pub struct DockerfileContainer {
    /// The location of the Dockerfile that defines the contents of the container.
    /// The path is relative to the folder containing the `devcontainer.json` file.
    pub dockerfile: String,
    /// The location of the context folder for building the Docker image.
    /// The path is relative to the folder containing the `devcontainer.json` file.
    pub context: Option<String>,
    /// Docker build-related options.
    pub options: Option<BuildOptions>,
}

pub struct BuildOptions {
    /// Target stage in a multi-stage build.
    pub target: Option<String>,
    /// Build arguments.
    pub args: Option<HashMap<String, String>>,
    /// The image to consider as a cache. Use an array to specify multiple images.
    pub cache_from: Option<StringOrArray>,
}

pub enum StringOrArray {
    String(String),
    Array(Vec<String>),
}

pub struct ImageContainer {
    /// The docker image that will be used to create the container.
    pub image: String,
}

pub struct ComposeContainer {
    /// The name of the docker-compose file(s) used to start the services.
    pub docker_compose_file: StringOrArray,
    /// The service you want to work on. T
    /// his is considered the primary container for your dev environment
    ///  which your editor will connect to.
    pub service: String,
    /// An array of services that should be started and stopped.
    pub run_services: Option<Vec<String>>,
    /// The path of the workspace folder inside the container.
    /// This is typically the target path of a volume mount in the docker-compose.yml.
    pub workspace_folder: String,
    /// Action to take when the user disconnects from the primary container in their editor.
    ///  The default is to stop all of the compose containers.
    pub shutdown_action: Option<ShutdownAction>,
    /// Whether to overwrite the command specified in the image. The default is false.
    pub override_command: Option<bool>,
}
