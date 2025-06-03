<template>
	<header class="top-0 z-10">
		<UContainer class="md:py-2">
			<UNavigationMenu
				:items="mobileItems"
				variant="link"
				:ui="{
					root: 'md:hidden'
				}"
			/>
			<UNavigationMenu
				:items="desktopItems"
				variant="link"
				:ui="{
					root: 'hidden md:flex',
					viewportWrapper: 'max-w-2xl absolute-center-h',
					list: 'md:gap-x-2'
				}"
			/>
		</UContainer>
	</header>
</template>

<script lang="ts" setup>
	const { pages } = usePages();
	const { showSidebar } = useSidebar();

	const mobileItems = ref<any[]>([
		[
			{
				avatar: {
					src: "~/assets/logo.svg",
					size: "xl",
					ui: {
						root: "bg-transparent"
					}
				},
				to: "/"
			}
		],
		[
			{
				icon: "lucide:menu",
				onSelect: () => showSidebar.value = true
			}
		]
	]);

	const desktopItems = ref<any[]>([
		[
			{
				avatar: {
					src: "~/assets/logo.svg",
					size: "xl",
					ui: {
						root: "bg-transparent"
					}
				},
				to: "/"
			}
		],
		pages,
		[
			{
				avatar: {
					src: "https://github.com/benjamincanac.png",
					size: "xl",
					ui: {
						root: "bg-transparent"
					}
				},
				diabled: true
			}
		]
	]);
</script>
