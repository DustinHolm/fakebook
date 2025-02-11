import { SEMRESATTRS_SERVICE_NAME } from "@opentelemetry/semantic-conventions";
import { OTLPTraceExporter } from "@opentelemetry/exporter-trace-otlp-http";
import { registerInstrumentations } from "@opentelemetry/instrumentation";
import { BatchSpanProcessor } from "@opentelemetry/sdk-trace-base";
import { NodeTracerProvider } from "@opentelemetry/sdk-trace-node";
import { PrismaInstrumentation } from "@prisma/instrumentation";
import { Resource } from "@opentelemetry/resources";

export function register() {
  const provider = new NodeTracerProvider({
    resource: new Resource({
      [SEMRESATTRS_SERVICE_NAME]: process.env["TRACING_SERVICE_NAME"],
    }),
  });

  provider.addSpanProcessor(
    new BatchSpanProcessor(
      new OTLPTraceExporter({
        url: process.env["TRACING_TRACE_URL"],
      })
    )
  );

  registerInstrumentations({
    tracerProvider: provider,
    instrumentations: [new PrismaInstrumentation()],
  });

  provider.register();
}
