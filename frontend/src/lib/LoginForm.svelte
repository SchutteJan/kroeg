<script lang="ts">
	import { onMount } from 'svelte'
	import { login } from '../api/session'

	function handleLoginResponse(response: Response) {
		if (response.ok) {
			location.href = '/'
		} else {
			// TODO: Handle responses
			alert('Login failed: ' + response.statusText)
		}
	}

	function handleSubmit(event: SubmitEvent) {
		if (!(event.target instanceof HTMLFormElement)) {
			return
		}
		event.submitter?.setAttribute('aria-busy', 'true')
		login(new FormData(event.target)).then(handleLoginResponse)
	}
	const isLocalhost = location.hostname === 'localhost'
	let dummyEmail = ''
	let dummyPassword = ''

	onMount(() => {
		if (isLocalhost) {
			dummyEmail = 'some.user@gmail.com'
			dummyPassword = 'somepassw0rdthatisok'
		}
	})
</script>

<form on:submit|preventDefault={handleSubmit}>
	<fieldset>
		<label>
			Email
			<input
				type="email"
				name="email"
				id="email"
				aria-label="Login"
				autocomplete="email"
				required
				value={dummyEmail}
			/>
		</label>
		<label>
			Password
			<input
				type="password"
				name="password"
				aria-label="Password"
				autocomplete="current-password"
				required
				minlength="8"
				value={dummyPassword}
			/>
		</label>
	</fieldset>
	<button type="submit">Login</button>
</form>
