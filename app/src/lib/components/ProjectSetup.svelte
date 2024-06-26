<script async lang="ts">
	import Button from './Button.svelte';
	import KeysForm from './KeysForm.svelte';
	import ProjectSetupTarget from './ProjectSetupTarget.svelte';
	import newProjectSvg from '$lib/assets/illustrations/new-project.svg?raw';
	import { Project, ProjectService } from '$lib/backend/projects';
	import DecorativeSplitView from '$lib/components/DecorativeSplitView.svelte';
	import { getContext } from '$lib/utils/context';
	import { BranchController } from '$lib/vbranches/branchController';
	import { platform } from '@tauri-apps/api/os';
	import { from } from 'rxjs';
	import { goto } from '$app/navigation';

	export let remoteBranches: { name: string }[];

	const project = getContext(Project);
	const projectService = getContext(ProjectService);
	const branchController = getContext(BranchController);
	const platformName = from(platform());

	let selectedBranch = '';
	let loading = false;

	async function setTarget() {
		if (!selectedBranch) return;
		loading = true;
		try {
			// TODO: Refactor temporary solution to forcing Windows to use system executable
			if ($platformName == 'win32') {
				project.preferred_key = 'systemExecutable';
				projectService.updateProject(project);
			}
			await branchController.setTarget(selectedBranch);
			goto(`/${project.id}/`);
		} finally {
			loading = false;
		}
	}
</script>

<DecorativeSplitView img={newProjectSvg}>
	{#if selectedBranch && $platformName != 'win32'}
		{@const [remoteName, branchName] = selectedBranch.split(/\/(.*)/s)}
		<KeysForm {remoteName} {branchName} />
		<div class="actions">
			<Button style="ghost" kind="solid" on:mousedown={() => (selectedBranch = '')}>Back</Button>
			<Button style="pop" kind="solid" {loading} on:click={setTarget}>Let's go!</Button>
		</div>
	{:else}
		<ProjectSetupTarget
			projectName={project.title}
			{remoteBranches}
			on:branchSelected={(e) => {
				selectedBranch = e.detail;
				// TODO: Temporary solution to forcing Windows to use system executable
				if ($platformName == 'win32') {
					setTarget();
				}
			}}
		/>
	{/if}
</DecorativeSplitView>

<style lang="postcss">
	.actions {
		margin-top: var(--size-20);
		text-align: right;
	}
</style>
