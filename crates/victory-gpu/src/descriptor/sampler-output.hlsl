[RootSignature("DescriptorTable(SRV(t0,numDescriptors=2)),DescriptorTable(Sampler(s0,numDescriptors=3)),UAV(u0)")]
[numthreads(1, 1, 1)]
void main();

Texture2D<float> target : register(t0);
Texture2D<float> decoy : register(t1);
SamplerState changed : register(s0);
SamplerState known_clamp : register(s1);
SamplerState known_wrap : register(s2);
RWByteAddressBuffer output : register(u0);

void main() {
    const float2 uv = float2(1.25, 0.5);
    output.Store3(0, uint3(round(255.0 * float3(target.SampleLevel(changed, uv, 0),
                                               target.SampleLevel(known_clamp, uv, 0),
                                               decoy.SampleLevel(known_wrap, uv, 0)))));
}
