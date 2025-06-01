<template>
	<LayoutTile
		title="Test"
		description="A simple test. This is to show my understanding"
	>
		<div class="space-y-6 md:space-y-8">
			<UStepper :items="items" class="w-full">
				<template #address>
					<Placeholder class="aspect-video">
						Address
					</Placeholder>
				</template>

				<template #shipping>
					<Placeholder class="aspect-video">
						Shipping
					</Placeholder>
				</template>

				<template #checkout>
					<Placeholder class="aspect-video">
						Checkout
					</Placeholder>
				</template>
			</UStepper>
		</div>
	</LayoutTile>
</template>

<script lang="ts" setup>
	definePageMeta({
		name: "Test",
		icon: "lucide:terminal",
		description: "A test",
		category: "builds"
	});

	const items = [
		{
			slot: "address" as const,
			title: "Address",
			description: "Add your address here",
			icon: "i-lucide-house"
		},
		{
			slot: "shipping" as const,
			title: "Shipping",
			description: "Set your preferred shipping method",
			icon: "i-lucide-truck"
		},
		{
			slot: "checkout" as const,
			title: "Checkout",
			description: "Confirm your order"
		}
	] satisfies StepperItem[];
</script>
